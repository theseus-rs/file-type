use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979508: FileFormat = FileFormat {
    id: 27_979_508,
    source_type: SourceType::Wikidata,
    name: "RIFF Palette File",
    extensions: &["pal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
