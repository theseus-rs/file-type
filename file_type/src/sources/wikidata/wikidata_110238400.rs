use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238400: FileFormat = FileFormat {
    id: 110_238_400,
    source_type: SourceType::Wikidata,
    name: "Screenwriter 6 file format",
    extensions: &["mmsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
