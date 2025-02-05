use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59714459: FileFormat = FileFormat {
    id: 59_714_459,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript File Format",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
