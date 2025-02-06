use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128034881: FileFormat = FileFormat {
    id: 128_034_881,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 20",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
