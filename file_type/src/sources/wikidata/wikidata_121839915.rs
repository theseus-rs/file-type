use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121839915: FileFormat = FileFormat {
    id: 121_839_915,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript File Format 2.1",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
