initSidebarItems({"enum":[["APIKeyLocation",""],["AdditionalProperties",""],["CookieStyle",""],["HeaderStyle",""],["IntegerFormat",""],["NumberFormat",""],["OAuth2Flow",""],["Parameter",""],["ParameterSchemaOrContent",""],["PathStyle",""],["QueryStyle",""],["ReferenceOr",""],["SchemaKind",""],["SecurityScheme","Defines a security scheme that can be used by the operations. Supported schemes are HTTP authentication, an API key (either as a header or as a query parameter), OAuth2’s common flows (implicit, password, application and access code) as defined in RFC6749, and OpenID Connect Discovery."],["StatusCode",""],["StringFormat",""],["Type",""],["VariantOrUnknown",""],["VariantOrUnknownOrEmpty",""]],"fn":[["is_false",""]],"struct":[["AnySchema",""],["ArrayType",""],["Components","Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object."],["Contact","Contact information for the exposed API."],["Discriminator","When request bodies or response payloads may be one of a number of different schemas, a discriminator object can be used to aid in serialization, deserialization, and validation. The discriminator is a specific object in a schema which is used to inform the consumer of the specification of an alternative schema based on the value associated with it."],["Encoding","A single encoding definition applied to a single schema property."],["Example",""],["ExternalDocumentation","Allows referencing an external resource for extended documentation."],["Header","The Header Object follows the structure of the Parameter Object with the following changes:"],["Info","The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience."],["IntegerType",""],["License","License information for the exposed API."],["Link","The Link object represents a possible design-time link for a response. The presence of a link does not guarantee the caller’s ability to successfully invoke it, rather it provides a known relationship and traversal mechanism between responses and other operations."],["MediaType",""],["NumberType",""],["OAuth2Flows",""],["ObjectType",""],["OpenAPI",""],["Operation","Describes a single API operation on a path."],["ParameterData","Describes a single operation parameter."],["PathItem","Describes the operations available on a single path. A Path Item MAY be empty, due to ACL constraints. The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available."],["RequestBody",""],["Response",""],["Responses",""],["Schema",""],["SchemaData",""],["Server","An object representing a Server."],["ServerVariable","An object representing a Server Variable for server URL template substitution."],["StringType",""],["Tag","Adds metadata to a single tag that is used by the Operation Object. It is not mandatory to have a Tag Object per tag defined in the Operation Object instances."]],"type":[["Callback","A map of possible out-of band callbacks related to the parent operation. Each value in the map is a Path Item Object that describes a set of requests that may be initiated by the API provider and the expected responses. The key value used to identify the callback object is an expression, evaluated at runtime, that identifies a URL to use for the callback operation."],["Content",""],["Paths","Holds the relative paths to the individual endpoints and their operations. The path is appended to the URL from the Server Object in order to construct the full URL. The Paths MAY be empty, due to ACL constraints."],["SecurityRequirement","Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the Security Schemes under the Components Object."]]});