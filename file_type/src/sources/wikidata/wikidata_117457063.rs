use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117457063: FileFormat = FileFormat {
    id: 117_457_063,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Encrypted Database File 1.1",
    extensions: &["mda", "mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
