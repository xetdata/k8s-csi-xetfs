use veil::Redact;
use tonic::Status;
use crate::proto::csi::v1::NodePublishVolumeRequest;

#[derive(Clone, Default, Redact, Eq, PartialEq)]
pub(crate) struct XetCSIVolume {
    pub(crate) volume_id: String,
    pub(crate) path: String,
    pub(crate) repo: String,
    pub(crate) commit: String,
    // don't log following
    #[redact]
    pub(crate) user: String,
    #[redact]
    pub(crate) pat: String,
    pub(crate) cas: Option<String>,
}

const VOLUME_CONTEXT_REPO_KEY: &str = "repo";
const VOLUME_CONTEXT_COMMIT_KEY: &str = "commit";
const VOLUME_CONTEXT_CAS_KEY: &str = "cas";
const SECRETS_USER_KEY: &str = "user";
const SECRETS_PAT_KEY: &str = "pat";

impl XetCSIVolume {
    pub(crate) fn validate(&self) -> bool {
        !self.path.is_empty() && !self.volume_id.is_empty() && !self.commit.is_empty() && !self.repo.is_empty()
    }
}

impl TryFrom<NodePublishVolumeRequest> for XetCSIVolume {
    type Error = Status;

    fn try_from(mut value: NodePublishVolumeRequest) -> Result<Self, Self::Error> {
        let repo = if let Some(repo) = value.volume_context.remove(VOLUME_CONTEXT_REPO_KEY) {
            repo
        } else {
            return Err(Status::invalid_argument("missing repo in volume context"));
        };
        let commit = if let Some(commit) = value.volume_context.remove(VOLUME_CONTEXT_COMMIT_KEY) {
            commit
        } else {
            return Err(Status::invalid_argument("missing commit in volume context"));
        };
        // user and pat are optional
        let user = value.secrets.remove(SECRETS_USER_KEY).unwrap_or_default();
        let pat = value.secrets.remove(SECRETS_PAT_KEY).unwrap_or_default();

        // cas is optional and stored as option
        let cas = value.volume_context.remove(VOLUME_CONTEXT_CAS_KEY);

        Ok(XetCSIVolume {
            volume_id: value.volume_id,
            path: value.target_path,
            repo,
            commit,
            user,
            pat,
            cas,
        })
    }
}
