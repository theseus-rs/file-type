use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121298941: FileFormat = FileFormat {
    id: 121_298_941,
    puid: "wikidata/121298941",
    name: "age",
    extensions: &["age"],
    media_types: &["http://www.wikidata.org/.well-known/genid/028d13312180600533e8423e63a85aa1"],
    internal_signatures: &[],
    related_formats: &[],
};
