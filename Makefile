
# Register a local version of this instance to be scrapped by prometheus
register: service/local-registration.json
	curl -XPUT --data @$< http://consul.service.consul:8500/v1/catalog/register

_deregistration:
	cat service/local-registration.json  | jq '{"Datacenter": "home", "Node": .Node, "ServiceID": .Service.ID}' > service/.deregister.json

deregister: _deregistration
	curl -XPUT --data @service/.deregister.json http://consul.service.consul:8500/v1/catalog/deregister
