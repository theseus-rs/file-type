use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344823: FileFormat = FileFormat {
    id: 28_344_823,
    source_type: SourceType::Wikidata,
    name: "Debian source control file",
    extensions: &["dsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
