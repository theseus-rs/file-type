use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58725633: FileFormat = FileFormat {
    id: 58_725_633,
    source_type: SourceType::Wikidata,
    name: "Adobe PostScript Font Metrics file",
    extensions: &["pfm"],
    media_types: &["application/x-font-pfm"],
    internal_signatures: &[],
    related_formats: &[],
};
