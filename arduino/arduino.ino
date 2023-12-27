#define GREEN_LED_PIN 2


class State {
  public:
    static String light;

    State() {
    }
};
String State::light = "off";

State AppState;

class Led {
  public:
    void turnOn() { 
      if (!Led::isOn()) {
        digitalWrite(GREEN_LED_PIN, HIGH);

        AppState.light = "on";
      }
    }

    void turnOff() { 
      if (Led::isOn()) {
        digitalWrite(GREEN_LED_PIN, LOW);

        AppState.light = "off";
      }
    }

  protected:
    boolean isOn() {
      return AppState.light == "on";
    }
};

Led GreenLed;

void setup() {
  Serial.begin(9600);
  pinMode(GREEN_LED_PIN, OUTPUT);

  Serial.print("Starting serial logs");

}


void loop() {
  if(Serial.available() > 0) {
    char action;
    action = (char) Serial.read();
    Serial.print(action);

    switch (action) {
      case '1':
        GreenLed.turnOn();
        break;

      case '0':
        GreenLed.turnOff();
        break;

      default:
        break;
    }

    Serial.print(AppState.light);

    delay(100); 
  }
  // GreenLed.turnOn();
  // Serial.print(AppState.light);
  // delay(1000); 
  // GreenLed.turnOff();
  // Serial.print(AppState.light);
}
