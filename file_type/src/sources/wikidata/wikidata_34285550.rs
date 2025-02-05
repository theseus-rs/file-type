use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34285550: FileFormat = FileFormat {
    id: 34_285_550,
    source_type: SourceType::Wikidata,
    name: "Perl header",
    extensions: &["ph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
