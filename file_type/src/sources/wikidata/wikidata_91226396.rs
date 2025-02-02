use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_91226396: FileFormat = FileFormat {
    id: 91_226_396,
    source_type: SourceType::Wikidata,
    name: "QuarkXPress Project 2016",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[],
    related_formats: &[],
};
