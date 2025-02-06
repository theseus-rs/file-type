use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121298941: FileFormat = FileFormat {
    id: 121_298_941,
    source_type: SourceType::Wikidata,
    name: "age",
    extensions: &["age"],
    media_types: &["http://www.wikidata.org/.well-known/genid/028d13312180600533e8423e63a85aa1"],
    signatures: &[],
    related_formats: &[],
};
