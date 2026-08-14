desktop:
    cargo run

android:
    cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build
    ./gradlew build
    ./gradlew installDebug
    adb shell am start -n com.andrey.mobile_gfx/.MainActivity
    adb logcat -s mytag | tee logs/android_debug.txt
