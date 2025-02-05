use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47519802: FileFormat = FileFormat {
    id: 47_519_802,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format (generic)",
    extensions: &["ppp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
