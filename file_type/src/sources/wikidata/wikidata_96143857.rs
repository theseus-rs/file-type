use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96143857: FileFormat = FileFormat {
    id: 96_143_857,
    source_type: SourceType::Wikidata,
    name: "SurferGrid format",
    extensions: &["grd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
