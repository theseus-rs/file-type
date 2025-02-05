use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51370168: FileFormat = FileFormat {
    id: 51_370_168,
    source_type: SourceType::Wikidata,
    name: "Postscript Support File",
    extensions: &["psf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
