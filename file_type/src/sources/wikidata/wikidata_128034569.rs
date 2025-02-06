use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128034569: FileFormat = FileFormat {
    id: 128_034_569,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 19",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
