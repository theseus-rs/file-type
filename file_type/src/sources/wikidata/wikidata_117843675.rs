use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843675: FileFormat = FileFormat {
    id: 117_843_675,
    source_type: SourceType::Wikidata,
    name: "Wicat file",
    extensions: &["ged"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
