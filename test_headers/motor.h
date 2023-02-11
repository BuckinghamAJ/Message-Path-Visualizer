#include sensor.h

struct GPSData {
    double latitude;
    double longitude;
    float altitude;
    float speed;
    float heading;
}

struct MotorControl {
    int motor_id;
    int speed;
    int direction;
    int status;
    sensor.SensorData: motor_sensor;
}

