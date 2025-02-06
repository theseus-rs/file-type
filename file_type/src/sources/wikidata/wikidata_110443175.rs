use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110443175: FileFormat = FileFormat {
    id: 110_443_175,
    source_type: SourceType::Wikidata,
    name: "Visual Basics MAK File",
    extensions: &["mak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
