@featureSet=publish
Feature: Publish a message
  As a PubNub user
  I want to publish messages
  So I can use PubNub

  @contract=simplePublish @beta
  Scenario: Publishing a message
    Given the demo keyset
    When I publish a message
    Then I receive successful response
