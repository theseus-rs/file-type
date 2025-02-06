use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904539: FileFormat = FileFormat {
    id: 29_904_539,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System output file",
    extensions: &["lst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
