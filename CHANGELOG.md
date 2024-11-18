# init release 0.1.0
- parse kv2
- serde support
- minor tests

# 0.1.2
- support for multiple root objects in documents
- removed serde_kv2 (handle that shit yourself it's a pain to make nice) match against class_field and all T::deserialize(data.1)