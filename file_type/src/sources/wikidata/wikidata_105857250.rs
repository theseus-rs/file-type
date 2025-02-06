use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857250: FileFormat = FileFormat {
    id: 105_857_250,
    source_type: SourceType::Wikidata,
    name: "RoboHelp data",
    extensions: &["hpr"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
