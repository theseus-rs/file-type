use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086310: FileFormat = FileFormat {
    id: 110_086_310,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2021",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
