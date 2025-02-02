use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59714459: FileFormat = FileFormat {
    id: 59_714_459,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript File Format",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
