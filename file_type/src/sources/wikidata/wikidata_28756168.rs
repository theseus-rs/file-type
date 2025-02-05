use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756168: FileFormat = FileFormat {
    id: 28_756_168,
    source_type: SourceType::Wikidata,
    name: "FWKCS SRT file",
    extensions: &["srt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
