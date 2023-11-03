# Neurons

For every neuron you can set its weight using `set_neuron_weight` function.
For every neuron (apart of the Dummy Neuron), [External Data Provider](../README.md#external-data-provider) has to be set for the Voting System, otherwise the `tally` operation will fail if the neuron is included in any of the layers in the system.
Often, if something is not set, the result is just `0` while in the past there would be an error raised. But due to a very uncomfortable process of debugging, the decision of suppressing errors has been made.

## Dummy Neuron

This is a template neuron. It returns a hardcoded value so it does not matter who votes for which submission.
It should be used only for testing purposes and can be used as a template neuron to create new neurons as described in [the custom neurons section section](../README.md#custom-neurons).

## Assigned Reputation Neuron

It calculates the voting power based on the voter's reputation which is stored somewhere else and manually assigned in the system using the External Data Provider.
Reputation category should be set for the voter using `set_user_reputation_category` or `set_user_reputation_categories`. If it is not, then the neuron will return `0` voting power.

## Prior Voting History Neuron

This neuron checks the voter's activity in previous rounds. For every round in which the voter participated, it adds a bonus to the result voting power.
The bonus for each round has to be set manually.

If the user has not been active in previous rounds or the round they have been active in have no assigned bonus, the result will be `0`.

### Example:
It is round 3 and the user `user001` voted in round 2 and 1, but user `user002` only voted in the round 1.
Round 1 has assigned bonus `0.2` and round 2 has assigned bonus `0.3`.
So, `user001` is going to have voting power of `0.5` in this neuron and `user002` `0.2`.

## Trust Graph Neuron

The idea is that every user will specify a list of trusted users. Underneath this neuron uses the page rank algorithm. Essentially most trusted users will get the highest bonus here.
