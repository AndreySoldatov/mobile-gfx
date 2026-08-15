desktop:
    cargo run

android-demo-dir := "android-demo"
android:
    cd {{android-demo-dir}} && cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build
    cd {{android-demo-dir}} && ./gradlew build
    cd {{android-demo-dir}} && ./gradlew installDebug
    adb shell am start -n com.andrey.android_demo/.MainActivity
    adb logcat -s mytag | tee logs/android_debug.txt
