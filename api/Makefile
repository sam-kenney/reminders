LOCATION ?= australia-southeast1


build:
ifndef GOOGLE_CLOUD_PROJECT_ID
	$(error "GOOGLE_CLOUD_PROJECT_ID must be set")
endif

	-gcloud builds submit \
		--project $(GOOGLE_CLOUD_PROJECT_ID) \
		--config cloudbuild.yaml \
		--substitutions=_GOOGLE_CLOUD_PROJECT_ID=$(GOOGLE_CLOUD_PROJECT_ID),_LOCATION=$(LOCATION)

deploy:
ifndef GOOGLE_CLOUD_PROJECT_ID
	$(error "GOOGLE_CLOUD_PROJECT_ID must be set")
endif

ifndef GOOGLE_CLOUD_SERVICE_ACCOUNT
	$(error "GOOGLE_CLOUD_SERVICE_ACCOUNT must be set")
endif

ifndef FIREBASE_URI
	$(error "FIREBASE_URI must be set")
endif

ifndef AUTH_TOKEN
	$(error "AUTH_TOKEN must be set")
endif

	-gcloud run deploy reminders-api \
		--image $(LOCATION)-docker.pkg.dev/$(GOOGLE_CLOUD_PROJECT_ID)/cloud-run-source-deploy/reminders-api \
		--service-account $(GOOGLE_CLOUD_SERVICE_ACCOUNT) \
		--project $(GOOGLE_CLOUD_PROJECT_ID) \
		--region $(LOCATION) \
		--allow-unauthenticated \
		--set-env-vars FIREBASE_URI=$(FIREBASE_URI) \
		--set-env-vars AUTH_TOKEN=$(AUTH_TOKEN) \
		--set-env-vars NO_COLOR=true \
		--port 9999
