use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207202: FileFormat = FileFormat {
    id: 28_207_202,
    source_type: SourceType::Wikidata,
    name: "QuickTime Image Format",
    extensions: &["qif", "qtif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
