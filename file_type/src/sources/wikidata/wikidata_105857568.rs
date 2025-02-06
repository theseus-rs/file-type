use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857568: FileFormat = FileFormat {
    id: 105_857_568,
    source_type: SourceType::Wikidata,
    name: "blueMSX machine settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    signatures: &[],
    related_formats: &[],
};
