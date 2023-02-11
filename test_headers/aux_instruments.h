#include sensor.h

struct RobotStatus {
    int battery_level;
    bool charging;
    bool error_flag;
    char error_message[256];
    sensor.SensorData: robot_sensor;
}


struct AudioData {
    int sample_rate;
    int bit_depth;
    int channels;
    int timestamp;
    unsigned char data[];
}

struct CameraFrame {
    int width;
    int height;
    int channels;
    int timestamp;
    unsigned char data[];
    sensor.SensorData: camera_sensor;
    AudioData: audio_capture;
}