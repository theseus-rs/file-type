use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207202: FileFormat = FileFormat {
    id: 28_207_202,
    source_type: SourceType::Wikidata,
    name: "QuickTime Image Format",
    extensions: &["qif", "qtif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
