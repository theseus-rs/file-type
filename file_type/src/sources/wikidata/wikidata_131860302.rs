use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131860302: FileFormat = FileFormat {
    id: 131_860_302,
    source_type: SourceType::Wikidata,
    name: "MNI transformation file format",
    extensions: &["xfm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
