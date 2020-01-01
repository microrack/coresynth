# Состав устройства

1. PWM выход управления высотой VCO
2. 2 дискретных выхода gate 1 и gate 2
3. Дискретный выход звуковой частоты digiosc
4. 10 кнопок: `S1`-`S8`, `shift`, `play`. Подключены как цифровые входы с подтяжкой к питанию.
5. 9 светодиодов: по одному над каждой кнопкой секвенсера (`L1`-`L8`) и `LRUN` над кнопкой `play`. Подключены по схеме чарлиплексинга на 4 цифровых выхода
6. Энкодер. Подключен как 2 цифровых входа (A/B) с подтяжкой к питанию.
7. Вход MIDI (UART RX 31250-8-N-1)
8. Отладочный порт (UART TX 115200-8-N-1)

# Режимы работы устройства

1. Режим секвенсера
2. Сервисный режим

После включения устройство находится в режиме секвенсера

# Режим секвенсера

## Режимы работы секвенсора

1. Остановлен (stop)
2. Запущен (run)
3. В режиме паузы (pause)

### Переход между режимами

* mode: stop, `play` press, `shift`: 0 => run
* mode: stop, `play` press, `shift`: 1 => pause
* mode: run, `play` press, `shift`: 0 => stop
* mode: run, `play` press, `shift`: 1 => run, restart sequence
* mode: pause, `play` press, `shift`: 0 => run
* mode: pause, `play` press, `shift`: 1 => stop

### Секвенсор остановлен
При нажатии на кнопку звучит нота, при нажатой кнопке можно менять высоту ноты с помощью энкодера. На светодиодах отображается выбранная нота (`L1`—`L7` отображают ноту от A до H, `L8` отображает повышение на полтона). При отпускании нота перестает звучать.

### Секвенсор запущен
### Секвенсор в режиме паузы
При зажатой кнопке можно менять высоту ноты. Нота не воспроизводится.
Короткое нажатие включает и выключает ноту из последовательности.

В режиме запуска происходит последовательная установка частоты на выбранные каналы осциллятора, если нота выключена, устанавливается 0 частота (на digiosc сигнал отсутствует).

## Работа энкодера в режиме секвенсора

По-умолчанию энкодер меняет Tempo: темп отображается в виде мигания `LRUN` со частотой 1/4 темпа.

При зажатой клавише `shift` и соответствующей клавише секвенции меняется функция энкодера:

### `S1` Pattern length

Меняется длительность паттерна (от 1 до 8), отображается светодиодами `L1`-`L8` (светятся N светодиодов).

### `S2` Swing

Меняется отношения интервала слабой/сильной доли (от 10% до 90%), отображается светодиодами `L1`-`L8` (светятся N светодиодов).

### `S3` Change gate time

Меняется процент заполнения gate (от 10% до 90%), отображается светодиодами `L1`-`L8` (светятся N светодиодов).

### `S4` Gate 1 divide/multiply
### `S5` Gate 2 divide/multiply

Переключается отношение между интервалом запуска gate и интервалом тактового сигнала

* `L1`-`L8` светятся (одновременно 1): 8 — 7 — 6 — 5 — 4 — 3 — 2 — 1
* `L1`-`L8` выключены: slave (gate открывается только на включенных шагах секвенсера/по событиям MIDI)
* `L1`-`L8` мигают (одновременно 1): 1/2 — 1/4 — 1/4 (* 2/3) — 1/8 — 1/8 (* 2/3) — 1/16.

### `S6` Oscillator mode

Переключается режим управления осцилляторами. Отображается светодиодами `L1`-`L4` (одновременно 1).

_seq — управление осциллятором от секвенсера_

| mode | VCO | Digiosc |
|---|---|---|
| 1 | seq | seq |
| 2 | seq | MIDI-2 |
| 3 | MIDI-2 | seq |
| 4 | MIDI-1 | MIDI-2 |

### `S7` DigiOsc mode

Переключается режим работы цифрового осциллятора:

* меандр, степень заполнения от 0 до 100%, отображается светодиодами `L1`-`L8` (светятся N светодиодов).
* после достижения уровня заполнения 100% переключается в режим псевдослучайного шума, отображается светодиодами `L1`-`L8` (мигают все).

# Сервисный режим

Переключение в сервисный режим осуществляется нажатием кнопки `S8` при зажатой `shift`.

Выход из режима осуществляется нажатием `play`.

## Сервисные функции

### `S1` Смена канала MIDI primary

Номер канала устанавливается энкодером, отображается светодиодами `L1`-`L8` (одновременно 1).
При установленном 0 канале  MIDI primary выключен, светодиоды не светятся.

### `S2` Смена канала MIDI secondary

Номер канала устанавливается энкодером, отображается светодиодами `L1`-`L8` (одновременно 1).
При установленном 0 канале  MIDI secondary выключен, светодиоды не светятся.

### `S3` Режим разделения клавиатуры

При удержании кнопки начинают мигать все светодиоды, ожидается MIDI событие по одному из каналов, после чего все светодиоды горят непрерывно.

### `S4` Октава MIDI primary

Смещение октавы устанавливается энкодером. Отсуствие смещения — светодиоды не светятся. Смещение в отрицательную сторону отображается светодиодами `L1`-`L4` (одновременно 1) в обратном порядке (`L4` — смещение на 1). Смещение в положительную сторону отображается светодиодами `L5`-`L8` (одновременно 1).

### `S5` Октава MIDI secondary

Смещение октавы устанавливается энкодером. Отсуствие смещения — светодиоды не светятся. Смещение в отрицательную сторону отображается светодиодами `L1`-`L4` (одновременно 1) в обратном порядке (`L4` — смещение на 1). Смещение в положительную сторону отображается светодиодами `L5`-`L8` (одновременно 1).

### `S6` Вкл/выкл внешний MIDI clock/transport

Задается энкодером, отображается светодиодами `L1`-`L2`.

1. Светодиоды не горят: выключен clock/transport
2. Горит `L1`, clock включен, transport выключен
3. Горит `L1` и `L2`, clock включен, transport включен

### `S7` VCO calibration

При нажатии на кнопку запускается процедура калибровки VCO.
