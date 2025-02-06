use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_89897874: FileFormat = FileFormat {
    id: 89_897_874,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 8",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
