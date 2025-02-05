use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95733736: FileFormat = FileFormat {
    id: 95_733_736,
    source_type: SourceType::Wikidata,
    name: "RealAudio Metafile",
    extensions: &["ram"],
    media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
    signatures: &[],
    related_formats: &[],
};
