run:
	flutter run \
		--dart-define REMINDERS_BASE_URL=$(REMINDERS_BASE_URL) \
		--dart-define AUTH_TOKEN=$(AUTH_TOKEN)

build-apk:
	-flutter build apk --release \
		--dart-define REMINDERS_BASE_URL=$(REMINDERS_BASE_URL) \
		--dart-define AUTH_TOKEN=$(AUTH_TOKEN)
	-rm -f ~/Downloads/Reminders.apk
	-cp build/app/outputs/flutter-apk/app-release.apk ~/Downloads/Reminders.apk

build-macos:
	-flutter build macos --release \
		--dart-define REMINDERS_BASE_URL=$(REMINDERS_BASE_URL) \
		--dart-define AUTH_TOKEN=$(AUTH_TOKEN)
