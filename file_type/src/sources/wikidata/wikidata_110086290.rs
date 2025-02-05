use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086290: FileFormat = FileFormat {
    id: 110_086_290,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2020",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
