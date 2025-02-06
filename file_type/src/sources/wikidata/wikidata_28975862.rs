use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975862: FileFormat = FileFormat {
    id: 28_975_862,
    source_type: SourceType::Wikidata,
    name: "OOGL Bezier Surface BBP",
    extensions: &["bbp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
