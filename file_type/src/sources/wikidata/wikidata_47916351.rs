use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47916351: FileFormat = FileFormat {
    id: 47_916_351,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher Vector Graphics",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
