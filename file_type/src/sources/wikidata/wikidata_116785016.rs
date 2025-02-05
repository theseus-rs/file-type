use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116785016: FileFormat = FileFormat {
    id: 116_785_016,
    source_type: SourceType::Wikidata,
    name: "ISU file",
    extensions: &["isu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
