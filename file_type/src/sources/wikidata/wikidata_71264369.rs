use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71264369: FileFormat = FileFormat {
    id: 71_264_369,
    source_type: SourceType::Wikidata,
    name: "Hippel COmpressed SOng module",
    extensions: &["hipc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
