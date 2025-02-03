use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116785016: FileFormat = FileFormat {
    id: 116_785_016,
    source_type: SourceType::Wikidata,
    name: "ISU file",
    extensions: &["isu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
