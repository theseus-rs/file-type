use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418333: FileFormat = FileFormat {
    id: 111_418_333,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge URL File",
    extensions: &["adobebridge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
