FROM python:latest

COPY app.py /app.py


CMD ["python", "app.py", "/data"]

