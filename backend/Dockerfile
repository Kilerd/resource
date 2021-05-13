FROM frostming/pdm:latest as builder

WORKDIR /app

WORKDIR /app/resource
COPY pdm.lock pyproject.toml ./
#RUN pdm install
RUN pdm export -f requirements >> requirements.txt
RUN ls

FROM python:3.9-slim

COPY  app /application/app
COPY --from=builder /app/resource/requirements.txt /application/requirements.txt

EXPOSE 8000

ENV TELEGRAM_BOT_SECRET_KEY ""
ENV TELEGRAM_RESOURCE_CHANNEL ""
ENV MONGO_URL ""

WORKDIR /application
RUN pip install -r requirements.txt
CMD ["uvicorn app.main:app"]