use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130241065: FileFormat = FileFormat {
    id: 130_241_065,
    source_type: SourceType::Wikidata,
    name: "Literate Idris source code file",
    extensions: &["lidr"],
    media_types: &["text/x-literate-idris"],
    signatures: &[],
    related_formats: &[],
};
