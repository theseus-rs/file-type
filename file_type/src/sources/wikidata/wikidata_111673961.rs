use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111673961: FileFormat = FileFormat {
    id: 111_673_961,
    source_type: SourceType::Wikidata,
    name: "Kingsoft Writer Template",
    extensions: &["wpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
