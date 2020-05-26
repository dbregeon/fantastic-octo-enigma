Feature: Receive Order

   As a trader, I want the OMS to process order creations so that I can place them in the market.

Scenario: Simple order creation
Given an OMS with:
| System       |
| Sequencer    |
| Acceptor     |
| Market 1     |
| Market 2     |
And the following instrument was received:
| Id | Market Id |
| 1  | M1-1      |
When an order is sent:
| Order Id | Instrument Id | Quantity | Side | Destination |
| 1        | 1             | 1000     | Sell | Market 1    |
And the OMS runs one iteration
Then the OMS sends a Pending notification:
| Order Id | Instrument Id | Quantity | Side | Destination |
| 1        | 1             | 1000     | Sell | Market 1    |
And the OMS sends a Place event to Market 1

Scenario: Simple order rejection
Given an OMS with:
| System       |
| Sequencer    |
| Creator |
| Market 1     |
| Market 2     |
When an order is sent:
| Order Id | Instrument Id | Quantity | Side | Destination |
| 1        | 1             | 1000     | Sell | Market 1    |
And the OMS runs one iteration
Then the OMS sends a Rejection notification:
| Order Id | Message | Quantity |
| 1        | Unknown Instrument |
And the OMS does not send a Place notification to Market 1

Scenario: Order creation and cancellation in one iteration
Given an OMS with:
| System       |
| Sequencer    |
| Creator |
| Market 1     |
| Market 2     |
And the following instrument was received:
| Id | Market Id |
| 1  | M1-1      |
When an order is sent:
| Order Id | Instrument Id | Quantity | Side | Destination |
| 1        | 1             | 1000     | Sell | Market 1    |
And a cancellation is sent for:
| Order Id |
| 1        |
And the OMS runs one iteration
Then the OMS sends a Pending notification:
| Order Id | Instrument Id | Quantity | Side | Destination |
| 1        | 1             | 1000     | Sell | Market 1    |
And the OMS sends a Cancelled notification:
| Order Id |
| 1        |
And the OMS does not send a Place notification to Market 1