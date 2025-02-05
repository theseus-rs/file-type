use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126194534: FileFormat = FileFormat {
    id: 126_194_534,
    source_type: SourceType::Wikidata,
    name: "MySQL View Definition Format",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
