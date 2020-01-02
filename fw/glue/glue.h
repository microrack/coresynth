#pragma once

#include <stdint.h>
#include "stm32f1xx_hal.h"
#include "cmsis_os.h"

void set_pwm(float value, TIM_HandleTypeDef* tim, uint32_t channel);

void system_reset();

// TODO for some reason, bindgen wont generate osKernelSysTickFrequency
const uint32_t TICKS_FREQ = osKernelSysTickFrequency;
