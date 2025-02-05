use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73513552: FileFormat = FileFormat {
    id: 73_513_552,
    source_type: SourceType::Wikidata,
    name: "Puppy Linux DotPup installer package",
    extensions: &["pup"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
